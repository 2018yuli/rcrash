// function-extended.js - ES Module for Deno Core

import extended from "./extended-base.js";
import is from "./is-extended.js";
import args from "./arguments-extended.js";

const { isArray, isObject, isString, isFunction } = is;
const { argsToArray } = args;

function spreadArgs(f, args, scope) {
    switch ((args || []).length) {
        case 0:
            return f.call(scope);
        case 1:
            return f.call(scope, args[0]);
        case 2:
            return f.call(scope, args[0], args[1]);
        case 3:
            return f.call(scope, args[0], args[1], args[2]);
        default:
            return f.apply(scope, args);
    }
}

function hitch(scope, method, ...args) {
    if (isString(method) && !(method in scope)) {
        throw new Error(`${method} property not defined in scope`);
    } else if (!isString(method) && !isFunction(method)) {
        throw new Error(`${method} is not a function`);
    }
    if (isString(method)) {
        return function (...callArgs) {
            const func = scope[method];
            if (isFunction(func)) {
                return spreadArgs(func, args.concat(callArgs), scope);
            } else {
                return func;
            }
        };
    } else {
        return function (...callArgs) {
            return spreadArgs(method, args.concat(callArgs), scope);
        };
    }
}

function applyFirst(method, ...args) {
    if (!isString(method) && !isFunction(method)) {
        throw new Error(`${method} must be the name of a property or function to execute`);
    }
    return function (...scopeArgs) {
        const scope = scopeArgs.shift();
        if (isString(method)) {
            const func = scope[method];
            if (isFunction(func)) {
                return spreadArgs(func, args.concat(scopeArgs), scope);
            } else {
                return func;
            }
        } else {
            return spreadArgs(method, args.concat(scopeArgs), scope);
        }
    };
}

function hitchIgnore(scope, method, ...args) {
    if (isString(method) && !(method in scope)) {
        throw new Error(`${method} property not defined in scope`);
    } else if (!isString(method) && !isFunction(method)) {
        throw new Error(`${method} is not a function`);
    }
    if (isString(method)) {
        return function () {
            const func = scope[method];
            if (isFunction(func)) {
                return spreadArgs(func, args, scope);
            } else {
                return func;
            }
        };
    } else {
        return function () {
            return spreadArgs(method, args, scope);
        };
    }
}

function hitchAll(scope, ...funcs) {
    if (!isObject(scope) && !isFunction(scope)) {
        throw new TypeError("scope must be an object");
    }
    if (funcs.length === 1 && isArray(funcs[0])) {
        funcs = funcs[0];
    }
    if (!funcs.length) {
        funcs = Object.keys(scope).filter(k => isFunction(scope[k]));
    }
    for (const fn of funcs) {
        scope[fn] = hitch(scope, scope[fn]);
    }
    return scope;
}

function partial(method, ...args) {
    if (!isString(method) && !isFunction(method)) {
        throw new Error(`${method} must be the name of a property or function to execute`);
    }
    return function (...callArgs) {
        const fullArgs = args.concat(callArgs);
        if (isString(method)) {
            const func = this[method];
            if (isFunction(func)) {
                return spreadArgs(func, fullArgs, this);
            } else {
                return func;
            }
        } else {
            return spreadArgs(method, fullArgs, this);
        }
    };
}

function curryFunc(f, execute) {
    return function (...args) {
        return execute
            ? spreadArgs(f, args, this)
            : function (...innerArgs) {
                return spreadArgs(f, args.concat(innerArgs), this);
            };
    };
}

function curry(depth, cb, scope) {
    let f = scope ? hitch(scope, cb) : cb;
    if (depth) {
        const len = depth - 1;
        for (let i = len; i >= 0; i--) {
            f = curryFunc(f, i === len);
        }
    }
    return f;
}

export default extended
    .define(isObject, {
        bind: hitch,
        bindAll: hitchAll,
        bindIgnore: hitchIgnore,
        curry: (scope, depth, fn) => curry(depth, fn, scope),
    })
    .define(isFunction, {
        bind: (fn, obj, ...rest) => spreadArgs(hitch, [obj, fn, ...rest]),
        bindIgnore: (fn, obj, ...rest) => spreadArgs(hitchIgnore, [obj, fn, ...rest]),
        partial: partial,
        applyFirst: applyFirst,
        curry: (fn, num, scope) => curry(num, fn, scope),
        noWrap: {
            f() {
                return this.value();
            }
        }
    })
    .define(isString, {
        bind: (str, scope) => hitch(scope, str),
        bindIgnore: (str, scope) => hitchIgnore(scope, str),
        partial: partial,
        applyFirst: applyFirst,
        curry: (fn, depth, scope) => curry(depth, fn, scope),
    })
    .expose({
        bind: hitch,
        bindAll: hitchAll,
        bindIgnore: hitchIgnore,
        partial: partial,
        applyFirst: applyFirst,
        curry: curry,
    });
