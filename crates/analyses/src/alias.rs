use crate::alias::steensgaard::{
    FieldBased, FieldInsensitive, InterProcedural, MergeDeallocArg, NopDeallocArg, Steensgaard,
};

pub mod andersen;
pub mod constraint;
pub mod steensgaard;
#[cfg(test)]
mod test;

pub type TaintResult = Steensgaard<FieldBased, MergeDeallocArg, InterProcedural>;
pub type AliasResult = Steensgaard<FieldInsensitive, NopDeallocArg, InterProcedural>;
pub type IntraAliasResult = Steensgaard<FieldInsensitive, NopDeallocArg, InterProcedural>;

pub fn taint_results(rust_program: &utils::rustc::RustProgram) -> TaintResult {
    Steensgaard::field_based(rust_program)
}

pub fn alias_results(rust_program: &utils::rustc::RustProgram) -> AliasResult {
    Steensgaard::field_insensitive(rust_program)
}

pub fn intra_alias_results(rust_program: &utils::rustc::RustProgram) -> IntraAliasResult {
    Steensgaard::field_insensitive(rust_program)
}

pub fn report_results(rust_program: &utils::rustc::RustProgram) {
    Steensgaard::<FieldBased, MergeDeallocArg, InterProcedural>::field_based(rust_program)
        .print_results()
}
