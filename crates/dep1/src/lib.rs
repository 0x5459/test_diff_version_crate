
pub use mytestcrate as mytestcrate_dep1;

pub fn append_dep1() {
    mytestcrate::append("dep1");
}

