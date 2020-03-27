@IsTest
class TestClass {

    @TestSetup
    static void makeData(){
        SomeClass.doThing();
    }

}