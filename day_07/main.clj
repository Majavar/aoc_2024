(require '[clojure.string :as str])

(defn || [a b] (parse-long (str a b)))
(def ops_1 [* +])
(def ops_2 [* + ||])

(defn valid?
  ([[res first & values] ops] (valid? res values ops first))
  ([res values ops acc]
    (if (empty? values)
      (= res acc)
      (if (> acc res)
        false
        (some #(valid? res (rest values) ops (% acc (first values))) ops)))))

(defn -main [& args]
  (let [
    values (->>
            (->
              args
              (first)
              (or "example")
              (slurp)
              (str/replace #"\:" "")
              (str/split-lines))
            (map #(mapv parse-long (str/split % #" "))))]
    (println "Part 1:" (reduce + (map first (filter #(valid? % ops_1) values ))))
    (println "Part 2:" (reduce + (map first (filter #(valid? % ops_2) values ))))))

(apply -main *command-line-args*)
