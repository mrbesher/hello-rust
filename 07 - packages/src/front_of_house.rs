// all modules, functions, structs, etc. are private
// by default. Items in a parent module can't use private
// items inside child modules, the opposite is not ture.
pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
