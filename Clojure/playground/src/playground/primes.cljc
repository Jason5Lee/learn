(ns playground.primes
  (:require [clojure.spec.alpha :as s]))

(defn sieve
  "Returns a lazy sequence of the items in coll for which item is not dividable by each previous item."
  [numbers]
  (when-let [prime (first numbers)]
    (lazy-seq
      (cons prime
            (->> (rest numbers)
                 (remove #(-> (mod % prime) zero?))
                 sieve)))))

(s/fdef sieve
        :args (s/cat :numbers (s/coll-of number?))
        :ret (s/coll-of number?))

(defn primes-until
  "Returns a lazy sequence of the primes smaller than end."
  [end]
  (sieve (range 2 end)))

(s/fdef primes-until
        :args (s/cat :end number?)
        :ret (s/coll-of number?))
