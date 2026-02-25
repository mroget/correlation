import numpy as np
import scipy.stats

def random(n):
	x = np.random.random(n)
	y = np.random.random(n)
	return x,y

def randint(n):
	x = np.random.randint(0,n,n)
	y = np.random.randint(0,n,n)
	return x,y

def to_str(x):
	s = str(float(x))
	if s == "nan":
		return "f64::NAN"
	if '.' not in s:
		s = s + "."
	return s

def name_method(s):
	if s == "pearson":
		return "pearsonr"
	if s == "spearman":
		return "spearmanr"
	if s == "kendall":
		return "kendalltau"


lines = int(input())
tasks = []
for i in range(lines):
	l = input().split(" ")
	tasks.append({"n":int(l[0]), "rand":l[1], "method":l[2], "repeat" : int(l[3])})

print("use correlation::*;\n\n\n\n")
for task in tasks:
	for i in range(task["repeat"]):
		x,y = (randint(task["n"]) if task["rand"] == "int" else random(task["n"]))
		if task["method"] == "spearman":
			res = scipy.stats.spearmanr(x,y)
		elif task["method"] == "kendall":
			res = scipy.stats.kendalltau(x,y)
		else:
			res = scipy.stats.pearsonr(x,y)
		print("#[test]\nfn {}_{}_{}_{}() ".format(task["method"], task["rand"], task["n"], i) + "{")
		print("""	let x = vec![{}];
	    let y = vec![{}];
	    let r : f64 = {};
	    let result = {}(&x,&y);
	    assert!((result.is_nan() && r.is_nan()) || (result - r).abs() <= 1e-7);""".format(",".join(list(map(to_str,x))),",".join(list(map(to_str,y))),to_str(res[0]), name_method(task["method"])))
		print("}\n\n")
	print("\n\n")

