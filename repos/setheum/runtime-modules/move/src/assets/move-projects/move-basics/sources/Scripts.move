script {
    fun generic_1<T: copy + drop>(x: T) {
        let _y = x;
    }
}

script {
    fun empty_loop() {
        let iterations: u64 = 10;

        while (iterations > 0) {
            iterations = iterations - 1;
        }
    }
}

script {
    fun empty_loop_param(iterations: u64) {
        while (iterations > 0) {
            iterations = iterations - 1;
        }
    }
}

script {
    use std::vector;
    fun empty_loop_param_with_a_vector(a: vector<u64>) {
        vector::pop_back(&mut a);
        let s = vector::pop_back(&mut a);

        while (s > 0) {
            s = s - 1;
        }
    }
}

script {
    fun empty_scr() {}
}
