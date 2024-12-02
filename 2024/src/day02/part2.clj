(ns day02.part2
  (:require [clojure.test :refer [deftest]]
            [day02.part1 :as part1]
            [ysera.test :refer [is is-not is=]]))


(defn safe?
  {:test (fn []
           (is (safe? [7 6 4 2 1]))
           (is-not (safe? [1 2 7 8 9]))
           (is-not (safe? [9 7 6 2 1]))
           (is (safe? [1 3 2 4 5]))
           (is (safe? [8 6 4 4 1]))
           (is (safe? [1 3 6 7 9])))}
  ([report direction last-level already-skipped]
   (or (empty? report)
       (let [[level & tail] report]
         (or (cond
               (nil? last-level)
               (safe? tail :unknown level already-skipped)

               (> level last-level)
               (and (not= direction :decreasing)
                    (<= (- level last-level) 3)
                    (safe? tail :increasing level already-skipped))

               (< level last-level)
               (and (not= direction :increasing)
                    (<= (- last-level level) 3)
                    (safe? tail :decreasing level already-skipped))

               :else false)
             (and (not already-skipped)
                  (safe? tail direction last-level true))))))
  ([report]
   (safe? report :unknown nil false)))


(defn solve
  [name]
  (->> (part1/process-input name)
       (filter safe?)
       (count)))


(deftest example
  (is= (solve ::example)
       4))


(deftest input
  (is= (solve ::input)
       674))
