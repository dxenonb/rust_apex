public class Foo { 
    global Helper foo;

    public Helper foo;

    private Helper foo;

    Helper foo;

    transient Helper foo;

    global Integer foo = 5;

    global static Integer foo = 5;
    static Integer foo = 5;

    global static final Integer foo = 5;
    static final Integer foo = 5;

    global final static Integer foo = 5;
    final static Integer foo = 5;

    final static Integer foo { get; set; }
    final static Integer foo { get; }
    final static Integer foo { set; }

    public final static Integer foo { get; private set; }
    public final static Integer foo { 
        get() {
            return 5;
        }

        set() {
            this.x = value;
        }
    }
}