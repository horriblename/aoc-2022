;; ------
;; Task 1
;; ------
(local max_cal
  (let [lines (io.lines :input.txt)]
    (accumulate [sm {:sum 0 :max 0} line lines]
      (let [num (tonumber line)]
        (if num
            {:sum (+ sm.sum num) :max sm.max}
            {:sum 0 :max (math.max sm.sum sm.max)})))))

(print max_cal.max)

;; ------
;; Task 2
;; ------

;; a "pseudo" min heap, 
(local Heap [0 0 0])

(fn Heap.swap [self a b]
  (let [tmp (. self a)]
    (do
      (tset self a (. self b))
      (tset self b tmp))))

(fn Heap.reorder [self] 
  (match self 
    (where [a b c] (and (< b a) (<= b c))) (self:swap 1 2)
    (where [a _ c] (< c a)) (self:swap 1 3)))

(fn Heap.update [self new_val]
  (do 
    (tset self 1 new_val)
    (self:reorder)))

(fn Heap.peek [self]
  (. self 1))

(local max_three
  (let [lines (io.lines :input.txt)]
    (accumulate [sm {:sum 0 :top_three Heap} line lines]
      (let [num (tonumber line)]
        (if num
            (do
              (tset sm :sum (+ sm.sum num))
              sm)
            (do
              (when (> sm.sum (sm.top_three:peek))
                (sm.top_three:update sm.sum))
              (tset sm :sum 0)
              sm))))))

(print 
  (accumulate [sum 0 _ v (ipairs max_three.top_three)]
    (+ sum v)))

