
// console.log(JSON.stringify(globalThis));
// console.log(nools);
// console.log(this);
var Message = function (message) {
    this.text = message;
};

console.log('22222222', nools);
console.log('3333', this.nools);
var flow = nools.flow("Hello World", function (flow) {

    //find any message that is exactly hello world
    flow.rule("Hello", [Message, "m", "m.text =~ /^hello\\sworld$/"], function (facts) {
        facts.m.text = facts.m.text + " goodbye";
        this.modify(facts.m);
    });

    //find all messages then end in goodbye
    flow.rule("Goodbye", [Message, "m", "m.text =~ /.*goodbye$/"], function (facts) {
        console.log(facts.m.text);
    });
});

console.log('33333333');


var session = flow.getSession();
//assert your different messages
session.assert(new Message("goodbye"));
session.assert(new Message("hello"));
session.assert(new Message("hello world"));
session.match();


//same as above getSession will assert the passed in objects
var session2 = flow.getSession(
    new Message("goodbye"),
    new Message("hello"),
    new Message("hello world")
);
session2.match();