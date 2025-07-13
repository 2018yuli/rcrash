import * as extd from "./extended.js";
import * as compile from "./compile.js";
import * as FlowContainer from "./flowContainer.js";

function isNoolsFile(file) {
    return (/\.nools$/).test(file);
}

function parse(source, sourceContent = null) {
    let ret;
    if (isNoolsFile(source)) {
        if (!sourceContent) {
            throw new Error("deno_core 环境中不支持 fs.readFileSync，请从 Rust 中读取后传入 sourceContent");
        }
        ret = compile.parse(sourceContent, source);
    } else {
        ret = compile.parse(source);
    }
    return ret;
}

export const Flow = FlowContainer;

export const getFlow = FlowContainer.getFlow;
export const hasFlow = FlowContainer.hasFlow;

export function deleteFlow(name) {
    FlowContainer.deleteFlow(name);
    return this;
}

export function deleteFlows() {
    FlowContainer.deleteFlows();
    return this;
}

export const flow = FlowContainer.create;

export function compileFlow(file, options = {}, cb) {
    if (typeof options === "function") {
        cb = options;
        options = {};
    }
    if (typeof file === "string") {
        options.name = options.name || (isNoolsFile(file) ? file.replace(/.*\/(.*)\.nools$/, "$1") : null);
        throw new Error("请使用 parse 方法先处理 sourceContent，再传入 compileFlow");
    }
    if (!options.name) {
        throw new Error("Name required when compiling nools source");
    }
    return compile.compile(file, options, cb, FlowContainer);
}

export function transpile(file, options = {}) {
    if (typeof file === "string") {
        options.name = options.name || (isNoolsFile(file) ? file.replace(/.*\/(.*)\.nools$/, "$1") : null);
        throw new Error("请使用 parse 方法先处理 sourceContent，再传入 transpile");
    }
    return compile.transpile(file, options);
}

export { parse };
