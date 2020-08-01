use notedown_ast::AST;

pub fn regroup_list_view(lists: &[(usize, &str, Vec<AST>)]) -> Vec<AST> {
    println!("{:#?}", lists);
    vec![]
}

pub fn regroup_table_view(table: &[Vec<Vec<AST>>]) -> Vec<AST> {
    for line in table {
        for item in line {
            println!("{:?}", item)
        }
    }
    vec![]
}