(defpackage poker-cl
  (:use :cl))
(in-package :poker-cl)

(defun gen_poker (num) (mapcar (lambda (tag) (format nil "~S~S" tag num)) (list "黑桃" "红心" "梅花" "街砖")))

(defun gen_two_poker ()
  (mapcar 'gen_poker (list 2 3 4 5 6 7 8 9 10 'j 'q 'k 'a)))

(mapcar 'print (gen_two_poker))
