(ns day02.part1
  (:require [clojure.test :refer [deftest]]
            [util :refer [get-resource-path]]
            [ysera.test :refer [is is-not is=]]))


(defn process-input
  {:test (fn []
           (is= (process-input ::example)
                [[7 6 4 2 1]
                 [1 2 7 8 9]
                 [9 7 6 2 1]
                 [1 3 2 4 5]
                 [8 6 4 4 1]
                 [1 3 6 7 9]]))}
  [name]
  (let [path (get-resource-path name)]
    (with-open [reader (clojure.java.io/reader path)]
      (->> (line-seq reader)
           (map #(clojure.string/split % #"\s+"))
           (map #(map ^[String] Integer/parseInt %))
           (map vec)
           (vec)))))


(defn safe?
  {:test (fn []
           (is (safe? [7 6 4 2 1]))
           (is-not (safe? [1 2 7 8 9]))
           (is-not (safe? [9 7 6 2 1]))
           (is-not (safe? [1 3 2 4 5]))
           (is-not (safe? [8 6 4 4 1]))
           (is (safe? [1 3 6 7 9])))}
  ([report direction last-level]
   (or (empty? report)
       (let [[level & tail] report]
         (cond
           (nil? last-level)
           (safe? tail :unknown level)

           (> level last-level)
           (and (not= direction :decreasing)
                (<= (- level last-level) 3)
                (safe? tail :increasing level))

           (< level last-level)
           (and (not= direction :increasing)
                (<= (- last-level level) 3)
                (safe? tail :decreasing level))

           :else false))))
  ([report]
   (safe? report :unknown nil)))


(defn solve
  [name]
  (->> (process-input name)
       (filter safe?)
       (count)))


(deftest example
  (is= (solve ::example)
       2))


(deftest input
  (is= (solve ::input)
       639))
