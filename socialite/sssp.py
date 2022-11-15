SRC = 1

`Edge(int a, int b, int c) indexby a.
 Edge(f, t, l) :- row=$read("/home/ubuntu/workspace/dataset/in/edge.facts"),
                  (f_s,t_s,l_s)=$split(row, "\t"), f=$toInt(f_s), t=$toInt(t_s), l=$toInt(l_s).`


`Path(int a:0..100, int dist).
 Path(t, $min(d)) :- t_s=$SRC, d=0;
                  :- Path(s, d1), Edge(s, t, l), d=d1+l.`
