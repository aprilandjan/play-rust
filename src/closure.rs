
use rand::Rng;

#[derive(Debug)]
struct TreeNode {
  pub id: i32,
  pub pid: i32,
  pub children: Vec<TreeNode>,
}

fn generate_node_list(size: i32) -> Vec<TreeNode> {
  let mut list: Vec<TreeNode> = vec![];
  for num in 1..size {
    let mut pid = 0;
    if num > 1 {
      pid = rand::thread_rng().gen_range(1..num);
    }
    list.push(TreeNode {
      id: num,
      pid,
      children: vec![],
    })
  }

  return list;
}

pub fn create_node_tree() {
  // let traverse = |node| {
  //   println!("traverse tree node...");
  // };
  let list = generate_node_list(30);
  println!("complete node list: {:?}", list);
  // TODO: convert the list into a tree
}