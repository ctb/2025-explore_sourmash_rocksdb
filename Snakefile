rule default:
    input: "hashes.sig.zip",

rule make_test_rocksdb:
    input: "sig-mf.csv",
    output: directory("tst.rocksdb"),
    shell: "sourmash scripts index sig-mf.csv -o {output}"

rule make_manifest_csv:
    input: expand("sketches/{F}.fa.sig", F=[1, 2, 47, 63]),
    output: "sig-mf.csv",
    shell: "sourmash sig collect -F csv -o {output} {input}"
    
rule extract_rocksdb:
    input:
        db="tst.rocksdb",
        src="src/main.rs",
    output: "hashes.txt"
    shell: "cargo run {input.db} -o {output} -s 10000"

rule make_sig:
    input: "hashes.txt",
    output: "hashes.sig.zip",
    shell: "./hashes-into-sig.py {input} -o {output}"
