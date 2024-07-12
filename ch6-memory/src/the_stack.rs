/**
The Stack.
The stack is often described by analogy. Think of a stack of dinner plates waiting
in the cupboard of a commercial kitchen. Cooks are taking plates off the stack to
serve food, and dishwashers are placing new plates on the top.

The unit (the plate) of a computing stack is the stack frame, also known as the
allocation record. You are probably used to thinking of this as a group of variables
and other data. Like many descriptions in computing, the stack and the heap are
analogies that only partially fit. Even though the stack is often compared by analogy
to a stack of dinner plates waiting in the cupboard, unfortunately, that mental picture
is inaccurate. Here are some differences:

The stack actually contains two levels of objects: stack frames and data.
The stack grants programmers access to multiple elements stored within it, rather than
the top item only. The stack can include elements of arbitrary size, where the implication
of the dinner plate analogy is that all elements must be of the same size. So why is the
stack called the stack? Because of the usage pattern. Entries on the stack are made in a
Last In, First Out (LIFO) manner.

The entries in the stack are called stack frames. Stack frames are created as function
calls are made. As a program progresses, a cursor within the CPU updates to reflect the
current address of the current stack frame. The cursor is known as the stack pointer.

---

Thought Experiment Mentioned Which Represents The Stack:
"Imagine a diligent, yet absurdly single-minded cook in a commercial kitchen.
The cook takes each table’s docket and places those in a queue. The cook has
a fairly bad memory, so each current order is written down a notebook. As new
orders come in, the cook updates the notebook to refer to the new order. When
orders are complete, the notebook page is changed to the next item in the queue.
Unfortunately, for customers in this restaurant, the book operates in a LIFO manner.
Hopefully, you will not be one of the early orders during tomorrow’s lunch rush.

In this analogy, the notebook plays the role of the stack pointer. The stack itself
is comprised of variable-length dockets, representing stack frames. Like stack frames,
restaurant dockets contain some metadata. For example, the table number can act as the
return address.

---

The stack’s primary role is to make space for local variables. Why is the stack fast?
All of a function’s variables are side by side in memory. That speeds up access.
*/

pub fn read_only_is_strong_password<T: AsRef<str>>(pwd: T) -> bool {
    pwd.as_ref().len() > 5
}

/// This implicit conversion strategy does have significant risks, though.
/// If a stringified version of the password variable needs to be created
/// multiple times in the pipeline, it would be much more efficient to require
/// an explicit conversion within the calling application. That way the String
/// would be created once and reused.
pub fn mutable_is_strong_password<T: Into<String>>(pwd: T) -> bool {
    pwd.into().len() > 5
}
