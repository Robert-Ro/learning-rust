#![allow(unused_variables)] // 忽略未使用的变量的属性标记
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // 发散函数，不返回任何值
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);
}
