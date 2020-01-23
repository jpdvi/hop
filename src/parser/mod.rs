trait Node {
    fn token_literal(&self) -> String;
}

struct Statement {
    node -> Box<Node>;
}

trait Expression {

}
