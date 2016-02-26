// RP 20160215   pdoc-1.rs Test
//
//! # k(l)eine Rust Dokumentation
//! 
//! RobiSys

//!
//! # Rust Grundlagen
//!
//! hier nun erste Information
//!
//! *  ['rust-doc'](https://www.rust-lang.org)    'The Rust Programming Language'
//!
//!    weiter
//!
//! *  ['Crates'](https://crates.io/)   'CARGO  Browse all Crates'
//!    weiter
//!
//! *  ['DOC'](https://doc.rust-lang.org/) 'Doc'  [Core] (doc.rustlang.org/nightly/core)
//!
//!    weiter
//!
//!
//! # Weitere Quellen
//!   
//! hier nun erste Information
//!
//! *  ['Rustbook'](http://rust-lang-de.github.io/rustbook-de/) 'Rustbook de'
//!    weiter
//!     
//! *  ['Rust nightly'](http://doc.rust-lang.org/nightly ) 'Rust Doku/'
//!  
//! *  [Wiki Rust] (https://de.wikipedia.org/wiki/Rust_(Programmiersprche) )  
//! 
//!     [ libraries games ](https://libraries.io/search?keywords=game¬languages=Rust)
//!     [ game of life ](https://github.com/lj-ditrapani/game-of-life-rust)
//!    
//! # 3 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter
//!    [TOML](https://github.com/alexcrichton/toml-rs)
//!
//!    [Hyperium-mime ](https://github.com/hyperium/mime.rs)
//!
//!    [markdown-editor](https://github.com/jbt/markdown-editor)
//!
//!     [Markdown-Editor](https://jbt.github.io/markdown-editor)
//!
//!   [opensuse Cargo](http://software.opensuse.org/package/cargo)
//!
//!
//!  # 4 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter7865
//!
//!
//! # 5 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter
//!
//!
//! # 6 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    
//!     Github
//!
//!    [github robisys ] (https://github.com/robisys)  
//!
//!    [github training] (https://training.github.com)
//!
//!    [github hilfe] (https://thomas-leister.de/allgemein/github-fuer-anfaenger-repository-anlegen-und-code-hochladen/)
//!
//!    [gitbuch](https://github.com/gitbuch/gitbuch_cc)
//!
//!    [ gitbuch html-version]         (http://gitbu.ch/pr01.html)
//!
//!    [GitHub Developer](https://developer.github.com/v3/)
//!
//!    [GitHub Gist](https://gist.github.com/)
//!
//! [github integration travis-ci] (https://github.com/integrations/travis-ci)
//!
//!  [ Travis-ci](https://travis-ci.org)
//!
//! # 7 Tools
//!  
//!    [ VUE Markdown-Editor ](http://vuejs.org/guide/installation.html)
//!
//!
//!
//! #  8 Tools
//!
//!    [Rust-mini bsp]  (https://github.com/schultyy/os_type )
//!
//!    [Travis-ci]  (https://docs.travis-ci.com )
//!   
//!  # Games
//!
//!    [Sokoban] (https://github.com/swatteau/sokoban-rs)
//!
//!
//!
//!
//!




//    #![doc(html_favicon_url = "https://www.rust-lang.org/favicon.ico") ]
//    #![crate_name = "stdd"]

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]



// #[doc(no_inline)] 
//pub use std;
//pub use std::env;
pub use std::option::Option;

    /// Person Struc
    ///
    ///hier dann weiteres
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}
    ///  Struc Rodel
    ///
    ///  weitere Info
pub struct Rodel {
    /// A Rodel must have a name, no matter how much Juliet may hate it
    ///
    /// weiter info für Rodel
    name1: String,
}


impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Example
    ///
    /// 
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// let person = Person::new("name");
    /// 
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

impl Rodel {
    /// Returns a Rodel with the name1 given them
    ///
    /// # Arguments
    ///
    /// * `name1` - A Teststring slice that holds the name of the person
    ///
    /// # Example
    ///
    ///
    /// // You Test can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// let Rodel = Rodel::new("name1"); 
    /// 
    pub fn new(name1: &str) -> Rodel {
        Rodel {
            name1: name1.to_string(),
        }
    }

    /// Gives a Testfriendly hello!  TestTestTestTest
    ///
    /// Says "Hello, [name1]" to the `Rodel` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name1);
    }
}
    /// Argumente
    pub fn argumente () {
    
    //let args: Vec<String> = env::args().collect();
    
    //println!("My path is {}.", args[0] );
    println!("I got ");
         
    }
// 
/*
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
# fn foo() {}
///
//
*/

/*
/// fn t ----- main Prog
fn t(nam) {
    /// Returns a Rodel with the name1 given them
    ///
    /// # Arguments
    ///
    /// * `nam` - A Teststring slice that holds the name of the person
    ///
      
}

*/

include!("lib1.rs");

//include!("unit_test.rs");

// include ("webbrowser.rs") ;

/*
#[cfg(not(test))]
fn main () {
   println!("If you see this, the tests were not compiled nor ran!");
}
*/

#[cfg(test)]
mod test {
     // help
     fn dd() {
     println!("function  dd ");

     }
     
   #[test]
   fn d() {
   println!("d -- I got test d ");
    
   }

   #[test]
//   #[should_panic]
    fn gg() {
    println!("gg -- I got gg ");
    

    dd();
    
        

    }
}

//  rp 20160130
// rustc --test main.rs

/*
/// Constructs a new `Rc<T>`.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
pub fn new(value: T) -> Rc<T> {
    // implementation goes here
}


*/

pub fn docbsp1() {
// RP 20160210 Test
//
//! # diese k(l)eine Rust Dokumentation
//! 
//! RobiSys

//!
//! # Rust Grundlagen
//!
//! hier nun erste Information
//!
//! *  ['rust-doc'](https://www.rust-lang.org)    'The Rust Programming Language'
//!
//!    weiter
//!
//! *  ['Crates'](https://crates.io/)   'CARGO  Browse all Crates'
//!    weiter
//!
//! *  ['DOC'](https://doc.rust-lang.org/) 'Doc'

}


/*

```
// This is a testable code block
```

```rust{.example}
// This is rust and also testable
```

```ignore
// This is not a testable code block
```

    // This is a testable code block (4-space indent)

```sh
# this is shell code and not tested
```


#[doc = "
Calculates the factorial of a number.

Given the input integer `n`, this function will calculate `n!` and return it.
"]

fn g() {
}



// Rustdoc will inline documentation of a `pub use` into this crate when the
// `pub use` reaches across crates, but this behavior can also be disabled.
#[doc(no_inline)]  
pub use std::option::Option;

/// A whizbang. Does stuff. (this line is the summary)
///
/// Whizbangs are ...
struct Whizbang;

 */
//

pub fn  pmain() {
    // console.log("start");


    let john = Person::new("John");

    john.hello();
    
    let jonas = Rodel::new("Johann");

    jonas.hello();
    
    //  console.log("stop");
    argumente();
    
}

//include!("webbrowser.rs");

/*
fn main() {
  pmain();
  }
*/  
  