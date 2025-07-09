async function hello() {
    return new Promise((res, _rej) => {
        Deno.core.print("Hello world!!!!!\n");
        console.log('1111');
        res("hello");
    });
}
hello();