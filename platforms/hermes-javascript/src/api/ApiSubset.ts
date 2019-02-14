import ffi from 'ffi'
import ref from 'ref'
import { Casteable } from '../casts'
import {
    SubscribeEventDescriptor,
    PublishEventDescriptor,
    MessageListener,
    FFIFunctionCall,
    HermesOptions
} from './types'

/* Tools */

const getMetadata = function<T = (SubscribeEventDescriptor | PublishEventDescriptor)>(
    obj: { [key: string]: T },
    eventName: string | number | symbol
) : T {
    if(typeof eventName === 'symbol')
        throw new Error('Symbol not expected')
    let metadata = obj[eventName]
    if(!metadata) {
        const matchingEntry = Object
            .entries(obj)
            .find(([key]) => typeof eventName === 'string' && eventName.startsWith(key))
        if(matchingEntry) {
            metadata = matchingEntry[1]
        } else {
            throw new Error(eventName + ' is not a known event!')
        }
    }
    return metadata
}

/* Class */

export default class ApiSubset {

    public call: FFIFunctionCall
    public destroy() {}
    private listeners = new Map()
    public options: HermesOptions
    protected facade: Buffer
    protected subscribeEvents: { [key: string]: SubscribeEventDescriptor }
    public publishEvents: { [key: string]: PublishEventDescriptor}
    public publishMessagesList: {[key: string]: any}
    public subscribeMessagesList: {[key: string]: any}

    constructor(protocolHandler: Buffer, call: FFIFunctionCall, options: HermesOptions, facadeName: string) {
        this.call = call
        this.options = options
        this.listeners = new Map()
        if(facadeName && protocolHandler) {
            const facadeRef = ref.alloc('void **')
            this.call(facadeName, protocolHandler, facadeRef)
            this.facade = facadeRef.deref()
        }
    }

    private makeSubscriptionCallback<T extends keyof this['subscribeMessagesList']>(eventName: T) {
        const {
            messageStruct,
            messageClass,
            dropEventName
        } = getMetadata(this.subscribeEvents, eventName)

        if(this.options.useJsonApi) {
            return ffi.Callback('void', [ ref.coerceType('string') ], (stringifiedJson: string) => {
                try {
                    const message = JSON.parse(stringifiedJson)
                    const actions = this.listeners.get(eventName)
                    actions.forEach(action => action(message))
                } catch (err) {
                    // eslint-disable-next-line
                    console.error(err)
                    throw err
                }
            })
        } else {
            return ffi.Callback('void', [ ref.refType(messageStruct) ], data => {
                try {
                    const message = new (messageClass || Casteable)(data)
                    const actions = this.listeners.get(eventName)
                    actions.forEach(action => action(message))
                    this.call(dropEventName, data)
                } catch (err) {
                    // eslint-disable-next-line
                    console.error(err)
                    throw err
                }
            })
        }
    }

    /**
     * Subscribes a message listener to a given hermes event.
     *
     * @param {*} eventName The event name to subscribe to.
     * @param {*} listener  A callback triggered when receiving a message.
     */
    on<T extends keyof this['subscribeMessagesList']>(eventName: T, listener: MessageListener<this['subscribeMessagesList'][T]>) {
        const {
            fullEventName,
            additionalArguments
        } = getMetadata(this.subscribeEvents, eventName)
        let listeners = this.listeners.get(eventName)
        if(!listeners) {
            listeners = []
            this.listeners.set(eventName, listeners)
            const callback = this.makeSubscriptionCallback(eventName)
            const args = [
                ...(additionalArguments && additionalArguments(eventName as string) || []),
                callback
            ]
            // Prevent GC
            process.on('exit', function() { callback })
            this.call(fullEventName + (this.options.useJsonApi ? '_json' : ''), this.facade, ...args)
        }
        listeners.push(listener)
        return listener
    }

    /**
     * Add a message listener that will only get called **once** for a given hermes event, then unsubscribe.
     * @param {*} eventName The event name to subscribe to.
     * @param {*} listener A callback triggered when receiving a message.
     * @returns {*} The reference to the wrapped listener.
     */

    once<T extends keyof this['subscribeMessagesList']>(eventName: T, listener: MessageListener<this['subscribeMessagesList'][T]>) {
        const listenerWrapper = (...args) => {
            this.off(eventName, listenerWrapper)
            listener(...args)
        }
        this.on(eventName, listenerWrapper)
        return listenerWrapper
    }

    /**
     * Removes an existing message listener for a given hermes event.
     *
     * @param {*} eventName The event name that was subscribed to.
     * @param {*} listener The reference to the listener callback to remove.
     */
    off<T extends keyof this['subscribeMessagesList']>(eventName: T, listener: MessageListener<this['subscribeMessagesList'][T]>) {
        const listeners = this.listeners.get(eventName)
        if(!listeners)
            return false
        const index = listeners.indexOf(listener)
        if(index < 0)
            return false
        listeners.splice(index, 1)
        return true
    }

    /**
     * Publish a message.
     */
    publish<T extends keyof this['publishEvents']>(eventName: T, message?: this['publishMessagesList'][T]) {
        const {
            messageClass,
            fullEventName,
            forgedStruct,
            forgeOptions
        } = getMetadata(this.publishEvents, eventName)

        if(message) {
            if(this.options.useJsonApi) {
                const cStringRef = ref.allocCString(JSON.stringify(message))
                this.call(fullEventName + '_json', this.facade, cStringRef)
            } else {
                const cDataRef = new (messageClass || Casteable)(message).forge(forgedStruct, forgeOptions).ref()
                this.call(fullEventName, this.facade, cDataRef)
            }
        } else {
            this.call(fullEventName, this.facade)
        }
    }
}