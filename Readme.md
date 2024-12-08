# A to Z of Rust
This is a collection of resources for learning the Rust programming language. 
#### For windows users you need VC++ linker, please install rust from official website.
#### Debugging rust programe is critical skill as it is a system level programming language. So make sure you debugging skill is great. 
#### for debugging you need to install VS code plugin rust-analyzer, codeLLDB and microsoft C++. Also other register level debugger would be a great choice. Make sure you allow breakpoint everywhere in your VS code settings. 
#### Use memory-view map extension to view memory map with address of your program. Tools are also known as memory profile tools such as vmmaps etc. MS Visual studio has best debugger tools with memory profiler for c++ programs. We can also use hexeditor for advnce coding. 
![Expert debugging video for rust](https://www.youtube.com/watch?v=TlfGs7ExC0A)
![Expert debugging video for rust1](https://www.youtube.com/watch?v=ebNMJ5rKRLU)
![Expert debugging video for rust2 memory layout](https://www.youtube.com/watch?v=7_o-YRxf_cc)
### Datatypes in Rust
Two major data types of rust are scaler types and compund types
1. Scaler types - such as interger float etc.    
2. Compond types - such as premitive - arrays and tuples and complex - Strings, vectors etc. 

### Functions in Rust
Functions are used to perform specific task. They can be called anywhere in program.
```rust
fn main() {
    println!("Hello World!");
}
```
In above example we have defined function named `main` which prints "Hello World!" on console.

### Variables in Rust
Variables are containers that store values. In rust variables are immutable by default but they can also be mutable.
let x = 5; //immutable variable
println!("{}",x);
let mut y = 10; //mutable variable
y += 1;
println!("{}",y);
In above example first line

### Ownership and memory management in Rust
Ownership is one of the most important concepts in Rust. It helps us manage memory efficiently without using garbage collector. Memory allocation happens when we create new value and deallocation occurs automatically when it goes out of scope or ownership changes hands.
```rust
fn main(){
    let s = String::from("hello"); //String type is allocated on heap so it has dynamic size at runtime
    takes_ownership(s); //takes_ownership function will take ownership of string s and free its memory after execution
}
fn takes_ownership(str:String){
    println!("{}",str);
}
```
In above example we have created a string variable `s` and passed it to another function `takes_ownership`. This means that now this function owns the string and once it finishes executing then only memory gets freed up. So if we try to access `s` again outside of this block then compiler will throw error saying that value was moved here and cannot be accessed anymore because it no longer exists. To avoid this problem we use references instead of passing actual values around since they don't own anything themselves just point towards something else already existing somewhere else in memory.   
**Ownership** has some rules in rust and we need to implement it correctly.    
lets see the correct algorithm for memory management in languages such as c or c++. This algorithm is known as control first approach and all the memory management is responsible upon programmer himself. 
1. Allocate memory dynamically using malloc().
2. Use pointer to refer to allocated memory location.
3. Free up allocated memory manually using free().
4. **Most important part is to assign NULL to the currently free memory address before assigning new one otherwise it may lead to dangling pointers.** Most programmers forget doing step number four which leads to memory leaks.    
In languages like python or java garbage collector takes care of freeing up unused memory automatically. But if you forcefully call the garbage collector explicitly then it may result in unexpected behavior. But in case of C/C++ we need to handle these things manually otherwise it might cause serious problems like memory leak or segmentation fault.
```python
import gc
class MyObj:
    def __init__(self):
        self.data = []
obj = MyObj()
del obj # delete object
print(obj.data) # Accessing deleted object results in AttributeError
gc.collect()  # Accessing deleted object results in AttributeError
```
To understand more clearly in terms of python programme about mutablity, think in terms of **inplace modification of objects.** In place modification is onlu possible in rust if the variable is declared as mutable using `mut` keyword and if the memory allocated to it is clearly defined either in stack or in heap. For heap allocation you will need special type of functions or traits(built-in function) to inplace modify the object. Dynamic data type or heap allocated memory always uses pointer references for changing the data inplace. Beacuse its heap allocated **ownership rules** will come into picture and accordingly the program need to written.             
Above code snippet shows how to properly release memory in Python. We have imported built-in module named gc which stands for garbage collector. Then we have defined class named MyObj having single attribute named data which holds list of integers. Next we instantiated an instance of MyObj class and assigned it to variable name obj. Finally we deleted obj object using del keyword followed by dot operator and finally printed content present inside data attribute of obj object. Since we did not call garbage collector explicitly hence interpreter automatically calls it during program termination phase thus ensuring proper cleanup of unused objects.
Lets see what happens when we run same thing in Java:

Above code snippet shows how to force garbage collection in python. Now lets see what happens when we run same thing in C/C++.

Below code snippet shows how to allocate memory dynamically in C/C++. Here we have declared integer pointer `ptr` and assigned it with return value of `malloc()` function which returns void* type i.e generic pointer capable enough to hold address of any datatype. Then we have called `free()` function which frees up previously allocated memory pointed by given argument. But what if we forgot to set null value after calling `free()`? Let's check below example.
```c
#include <stdlib.h>
int main(void){
    int *ptr = malloc(sizeof(int));
    *ptr = 10; // assign value 10 to ptr
    printf("%d\n",*ptr);// print value stored inside ptr
    free(ptr);
    *ptr = NULL; // Make sure you remove dangling pointer by assigning NULL value to it. Most developer will miss this step resulting in undefined behavior.

}
```
Above code snippet shows how to properly release memory in C/C++. We have initialized integer pointer `ptr` and assigned it with return value of `malloc()` function which returns void* type i.e generic pointer capable enough to hold address of any datatype. Then we have called `free()` function which frees up previously allocated memory pointed by given argument. After releasing memory we have set null value to current pointer so next time whenever someone tries accessing this particular piece of memory then system throws exception saying invalid read/write operation occurred due to uninitialized variable.      
But in case of rust we do not need to worry about freeing up memory ourselves because rust handles everything internally through concept known as **ownership**.
Now lets see how does it work in rust:
1. When we declare any variable like `let x = 5`, rust allocates required amount of bytes on stack and stores value there itself.
2. If we want to pass this variable into other functions then we simply copy its contents onto newly created space on stack and pass reference(pointer) pointing towards original location where our initial value resides.
3. Once control returns back from callee function then all local variables get destroyed
4. And finally when entire program completes execution then all global/static variables alongwith their respective memories gets released too.
### Stacks, Heap and ownership
**Concept of ownership we are dealing with mainly heap memory area. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.**  
[The best resoursesource I found for understanding ownership is by rust documentation](https://doc.rust-lang.org/beta/book/ch04-01-what-is-ownership.html).
#### Ownership rules for rust
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.   
![Best video to understand ownership in Rust](https://www.youtube.com/watch?v=Jcbh07P0jHU)
Stacks and heaps both are regions of computer memory used for storing data temporarily while performing certain operations. Stack is LIFO(last in first out) structure whereas heap follows FILO(first in last out). Both stacks and heaps follow push/pop semantics meaning whatever item pushed onto topmost layer remains accessible until popped off again. However unlike stack heap allows random access to elements residing within it making them suitable choice for implementing various algorithms requiring frequent lookups or modifications.
![stack](https://media.geeksforgeeks.org/wp-content/uploads/memoryLayoutC.jpg)
Image credit to GeetksforGeeks.  
Difference between stack and heap lies mainly in way they grow/shrink over time depending upon usage patterns. While growing upwards towards higher addresses stack grows downwards starting from highest available address till lowest possible limit reached by OS kernel. On contrary heap starts growing upwards beginning from lowest available address reaching maximum allowed limit imposed by operating system. Due to this difference in growth direction both stacks and heaps require different strategies for allocating/freeing up chunks of contiguous blocks of memory needed during runtime.
For simplicity sake imagine following scenario involving two threads trying simultaneously modify shared resource located somewhere deep down inside heap region.
ThreadA wants to increase counter variable stored inside heap whereas ThreadB wishes to decrease said counter. Without synchronization mechanism implemented either thread could potentially overwrite each others writes leading to inconsistent state eventually causing application crash. To prevent such situations occurring we must ensure mutual exclusion among concurrent accesses happening concurrently across multiple threads working together cooperatively toward achieving common goal.    
**Ownership can be transfered back using tuple or clone or reference.** Tuple is a very tedious way of transferring ownership, so we prefer cloning. The Clone method makes a deep copy of the heap data hence it is often expensive method.    
Reference is a good option when we want to share ownership but not necessarily modify it. This reference transfer is also known as borrow operations. Also we need to make sure this borrow operation happens without and damage to original variable.  Reference gives us ability to access shared resource safely without worrying about potential race conditions arising due to parallelism issues faced commonly encountered in multi-threaded environments.
So basically there are three ways to transfer ownership between entities namely tuple,clone and reference.         
To achieve this purpose rust provides three keywords namely borrow, move and clone which help us decide whether particular piece of information should reside solely owned by single entity or shared amongst several entities based upon context requirements. Borrow keyword indicates temporary sharing relationship between borrower and lender wherein former borrows latter’s resource temporarily without taking full ownership rights away from owner permanently. Move keyword signifies permanent transfer of ownership right from source entity to destination entity thereby invalidating previous holder completely. Clone keyword creates exact replica of existing object leaving original intact behind allowing independent modification without affecting parent copy whatsoever.
Let me show you few examples demonstrating usage of these keywords in practice.
Example 1 : Borrow Keyword
```rust
fn main(){
    let mut x = vec![1,2];
    let y = &mut x[0];
    *y+=1;//increment element at index zero
    println!("{:?}",x);//prints [2,2]
}
```
Here we have created vector containing two elements `[1,2]` and borrowed second element via reference `y`. Note that although we modified underlying value indirectly through dereferencing operator however original container still retains its integrity intact implying successful implementation of borrowing mechanism. However note carefully that we cannot mutate whole vector directly since it violates principle of immutability enforced by rust compiler. Instead we need to explicitly request permission from owner by prefixing expression with ampersand symbol indicating desire to acquire exclusive lock on target resource preventing simultaneous access attempts made by other parties involved.
Example 2 : Move Keyword    
```rust
fn main(){
    let mut x = vec![1,2];
    let y = x.clone();//create shallow copy of vector
    x.push(3);//push additional element into cloned version
    println!("{:?},{:?}",x,y);//prints [[1,2],[1,2]]
}
```
Here we have cloned entire vector including its contents and appended extra entry into copied variant leaving original untouched. Notice that even though we performed mutation operation on cloned version yet nothing happened to parent copy suggesting successful separation of concerns achieved thanks to cloning feature provided by rust standard library. Again observe closely that we were unable to append new element directly onto original vector owing to violation of immutability constraint imposed by rust compiler. Henceforth we had to resort fallback strategy involving creation of fresh instance altogether avoiding direct interference with pristine condition maintained throughout lifetime spanned by primary actor engaged in ongoing transactional activity.
Example 3 : Clone Keyword
```rust 
fn main(){
    let mut x = vec![1,2];
    let y = x.clone();
    x.push(3);
    println!("{:?},{:?}",x,y);//prints [[1,2],[1,2]]
}
```
Here we have demonstrated basic functionality offered by clone method belonging to Vec<T> trait. As expected output confirms presence of duplicate entries corresponding to identical copies generated independently without interfering with master branch controlled exclusively by sole proprietorship exercised collectively amongst participating stakeholders actively contributing towards overall success story unfolding progressively ahead.
Note that despite factually being able to distinguish between distinct identities manifested physically represented visually via graphical interface presented interactively displayed visually appealing manner nonetheless technically speaking those differences remain superficial reflecting mere surface level distinctions rather than profound structural variations inherent underneath hidden layers concealing intricate details governing inner workings encapsulated securely beneath opaque veil shielding curious minds yearning unconditionally seeking truth about nature of reality itself.      
#### Rules and regulations of passing ownership in Rust
1. When you borrow for reading the volue operations, its allowed easily.      
2. You cannot borrow twice a same mutable varible in a program, it leads to borrowing twice error. This error is important because mutable varible with more than one mutable borrowing will lead to race-around conditions. Hence its not allowed in rust and often lead to `second mutable borrowing error`.    
3. As long as your data is getting read its not an issue, but the moment when multiple people will try to write on your same piece of data, this will create a problem called race-around condition.    
#### Reference rule of rust
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
Multiple mutable reference are allowed, but you need to make sure to syncronize it while writing on it. 


### Control Flow Statements in Rust
## Books and Resources(more updates coming soon):
##### [1] Book: The Rust Programming Language (Second Edition)
##### [2] Rust by Example
##### [3] YouTube Playlist: Learn Rust in 10 minutes
##### [4] Too Many Lists - Chapter 1
##### [5] Too Many Lists - Chapter 2

## References(more updates coming soon)
##### [1] https://doc.rust-lang.org/book/second-edition/
##### [2] http://rustbyexample.com/index.html
##### [3] https://www.youtube.com/watch?v=ZI0hE4jOa-8&list=PLza5oFLQDxk7s6f9eYXgqyKJzLmUwBvG_
##### [4] https://github.com/rust-unofficial/too-many-lists/blob/master/_book/src/chapter_1.md
##### [5] https://github.com/rust-unofficial