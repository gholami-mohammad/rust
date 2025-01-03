// if we need some helper functions or utils, we can create a module inside test folder
// for example helpers and create a mod.rs file there.
// then we can add the module declaration in order_test.rs and any other files that we need it
// because this test file is not in root of tests directory, cargo wont consider it as a test
// so we can use it as a common module to set up our tests

pub fn setupConfig() {
    // do set up here
}
