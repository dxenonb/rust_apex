public class Foo {
    void doStuff() {
        if (true) ;

        if (true) {}

        if (true)
            return bar;
        
        if (true) {
            x += 5;
            return bar;
        }
    }
}