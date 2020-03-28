global class Foo {

    global static void assignments() {
        apples = 5;
        this.apples = 5;

        x += 1;
        x -= 'hey';
        x *= false;
        x /= null;

        x++;
        x--;

        ++x;
        --x;

        y = a || b && c;
    }

    global static void ternary() {
        x = true ? 1 : 0;
    }

    global static void calls() {
        foo();
        bar(x, y, z);
        baz(x, 1000, 'ninja stars', null);

        ApexPages.currentPage().getParameters().get('parameter');
    }

    global static void constructors() {
        new Apple();
        new Apple(x);
        new Apple(x, y);
        new Apple(a, b, '1000');
        new Map<Integer, List<Custom__c>>(x, y, z);
    }

}