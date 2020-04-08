@IsTest
public with sharing class Test {
    @IsTest
    public static Account fooBar() {
        // the "//" were being read as a comment (at time of writing SOQL interior is just ignored)
        a = [ SELECT Id, Website FROM Account WHERE Website = 'https://www.example.com' ];
    }
}