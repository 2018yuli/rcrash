// number-extended.js - ES Module for deno_core

import extended from "./extended.js";
import is from "./is-extended.js";

const mathPow = Math.pow;
const mathCeil = Math.ceil;

/**
 * 向上舍入并保留指定位数的小数（带增量控制）
 * @param {number} number 
 * @param {number} places 
 * @param {number} [increment=1e-20]
 * @returns {number}
 */
function round(number, places, increment = 1e-20) {
    const factor = 10 / (10 * (increment || 10));
    return (mathCeil(factor * +number) / factor).toFixed(places) * 1;
}

/**
 * 向上舍入保留指定位数的小数（纯幂运算）
 * @param {number} number 
 * @param {number} places 
 * @returns {number}
 */
function roundCeil(number, places) {
    const powed = mathPow(10, places);
    return mathCeil(number * powed) / powed;
}

const number = {
    round,
    roundCeil
};

const numberExtended = extended.define(is.isNumber, number).expose(number);
export default numberExtended;
