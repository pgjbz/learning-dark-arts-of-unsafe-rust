use crate::vecnomicon::VecNomicon;

mod vecnomicon;

fn main() {
    let mut vecnom: VecNomicon<i32> = VecNomicon::new();
    vecnom.push(10);
    vecnom.push(20);
    vecnom.push(30);
    vecnom.push(40);
    ops(vecnom);
}

fn ops(vecnom: VecNomicon<i32>) {}
