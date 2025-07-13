// arguments-extended.js - ES Module for deno_core

import extended from "./extended.js";
import is from "./is-extended.js";

function defineArgumentsExtended(extended, is) {
    const isArguments = is.isArguments;

    function argsToArray(args, slice = 0) {
        const ret = [];
        for (let i = slice; i < args.length; i++) {
            ret.push(args[i]);
        }
        return ret;
    }

    return extended
        .define(isArguments, {
            toArray: argsToArray
        })
        .expose({
            argsToArray: argsToArray
        });
}

const argumentsExtended = defineArgumentsExtended(extended, is);
export default argumentsExtended;
