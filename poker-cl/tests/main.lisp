(defpackage poker-cl/tests/main
  (:use :cl
        :poker-cl
        :rove))
(in-package :poker-cl/tests/main)

;; NOTE: To run this test file, execute `(asdf:test-system :poker-cl)' in your Lisp.

(deftest test-target-1
  (testing "should (= 1 1) to be true"
    (ok (= 1 1))))
