global with sharing class Calculator {

    private Double ans;

    public Calculator() {
        this.ans = 0;
    }

    public Calculator(Double ans) {
        this.ans = ans;
    }

    public Calculator add(Double val) {
        this.ans += val;
        
        return this;
    }
}