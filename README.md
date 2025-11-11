# Detailed Explanations of Implementation 

1. Import Statement (contract_b::ContractB)

Purpose: Import the ContractB struct from the contract_b module
Pattern: Same as the line above it for ContractA
Why: You need to import types before using them in your code

2. Public Attribute (public)

Purpose: Makes the trait implementation accessible externally
Syntax: #[public] is a Stylus SDK attribute macro
Why: Without this, the IContractA trait methods won't be callable from outside the contract

3. Function Call (self.contract_b.ret_numb())

Purpose: Call the ret_numb() method on the contract_b field and return its value (U256::from(4))
Structure:

self.contract_b accesses the ContractB instance stored in Foo
.ret_numb() calls the method defined in ContractB


Why: This demonstrates inheritance/composition by delegating to the child contract

4. Implements Attribute (implements(IContractA, IContractB))

Purpose: Tells Stylus that this impl block satisfies both trait implementations
Order: IContractA first, then IContractB (as specified in the hint)
Why: This enables the contract to expose methods from both traits as external functions that can be called by users

How It Works
This contract demonstrates composition-based inheritance:

Foo contains instances of both ContractA and ContractB
Traits (IContractA and IContractB) define interfaces
Implementations delegate to the composed contracts
The final impl block with #[implements(...)] exposes all trait methods plus additional methods (like foofoo)

When deployed, users can call:

ret_num_a() → returns 8 (from ContractA)
ret_num_b() → returns 4 (from ContractB)
foofoo() → returns 8 (calls ret_num_a() internally)

# More Information

Stylus Rust SDK Inheritance - Explained
This documentation describes how inheritance works in the Stylus Rust SDK for writing Arbitrum smart contracts in Rust. Let me break down the key concepts:
Core Concepts

1. Basic Inheritance Pattern
Instead of traditional Rust composition, Stylus uses macros to create Solidity-style inheritance:

#[public] - Marks a type as having public methods (provides the Router trait)
#[inherit(TypeA, TypeB)] - Makes the current type inherit from other types
#[entrypoint] - Marks the main contract type (where execution begins)

2. The Borrow Requirement
When a type inherits from another, it must be able to borrow the inherited type's data. Two ways to do this:
Option A - Automatic (recommended):

```rust
sol_storage! {
    #[entrypoint]
    pub struct Token {
        #[borrow]  // This annotation does it for you
        Erc20 erc20;
    }
}
```

Option B - Manual: Implement the Borrow<InheritedType> trait yourself.
Method Resolution (How Function Calls Work)
When you call a function on a contract, Stylus searches for it using Depth-First Search in this order:
Example:

```rust
#[public]
#[inherit(B, C)]  // Inheritance order matters!
impl A { pub fn foo() {} }

#[public]
impl B { pub fn bar() {} }

#[public]
impl C { 
    pub fn bar() {}  // ⚠️ This will NEVER be called FUNCTION FOUND FIRST WINS
    pub fn baz() {} 
}
```

Call Resolution:
foo() → Found in A → Executes A.foo()
bar() → Not in A, found in B → Executes B.bar() (C.bar is shadowed)
baz() → Not in A or B, found in C → Executes C.baz()

Method Overriding
Higher-level types automatically override methods in inherited types:
```rust
#[public]
#[inherit(B)]
impl A {
    pub fn foo() {}  // This overrides B.foo()
}

#[public]
impl B {
    pub fn foo() {}  // ⚠️ Unreachable when called through A
}
```

### Important Warning

Unlike Solidity, there are no override or virtual keywords in Stylus yet. You must manually ensure you're intentionally overriding functions—accidental shadowing can cause bugs.

Use Case: Extending Standards
This pattern lets you import a standard implementation (like ERC-20) and customize specific methods:

```rust
#[public]
#[inherit(Erc20)]  // Use standard ERC-20 implementation
impl MyToken {
    // Add custom methods
    pub fn mint(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        // Your custom logic
    }
    
    // Or override existing ones
    pub fn transfer(...) -> Result<(), Vec<u8>> {
        // Your modified transfer logic
    }
}
```

### Key Takeaways
* Inheritance order matters - Earlier types shadow later ones
* Use #[borrow] for easy storage setup
* Method search is depth-first
* No explicit override markers - be careful with naming
* First match wins - shadowed methods become unreachable
This system gives you Solidity-like inheritance patterns while working in Rust's type-safe environment!
