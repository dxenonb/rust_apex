@isTest
private class RunAsTest {
    @isTest
    private void fooBar() {
        system.runas ( getUser() ) {}

        System.runAs(getUser()) {

        }

        System.runAs(getUser()) {
            System.debug('Foo bar!');
        }
    }
}