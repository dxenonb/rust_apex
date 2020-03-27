public class Foo {
    // The parser does not yet support actual SOQL - it just looks for []

    Foo() {
        this.x = [this is a test [] foo];
    }
}