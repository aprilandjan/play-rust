
use rand::Rng;

#[derive(Debug)]
struct Node {
  pub id: i32,
  pub pid: i32,
}

#[derive(Debug)]
struct  TreeNode {
  pub id: i32,
  pub children: Vec<TreeNode>
}

fn generate_node_list(size: i32) -> Vec<Node> {
  let mut list: Vec<Node> = vec![];
  for num in 1..size {
    let mut pid = 0;
    if num > 1 {
      pid = rand::thread_rng().gen_range(1..num);
    }
    list.push(Node {
      id: num,
      pid,
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

  let mut trees: Vec<TreeNode> = vec![];
  for node in list.iter() {
    // if a node cannot find its parent in the list
    // then it should be a tree root node
    let parent = list.iter().find(|n| {
      n.id == node.pid
    });

    // put root into trees list
    match parent {
        Some(p) => {
          println!("find parent node for node #{:?}", node);
          // FIXME: should iterate through all deps
          let parent_tree = trees.iter_mut().find(|t| {
            t.id == node.pid
          });
          // FIXME: too nested
          match parent_tree {
            Some(pt) => {
              pt.children.push(TreeNode { id: node.id, children: vec![] })
            },
            None => {

            }
          }
        },
        None => {
          println!("cannot find parent node for node #{:?}, move it as the root tree node", node);
          trees.push(TreeNode { id: node.id, children: vec![] });
        },
    }
  }
  println!("the node trees are as followed: {:?}", trees);
}