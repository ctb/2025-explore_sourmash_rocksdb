#! /usr/bin/env python
import sys
import argparse
import sourmash
from sourmash import sourmash_args


def main():
    p = argparse.ArgumentParser()
    p.add_argument('infile')
    p.add_argument('-o', '--outfile-sig', required=True)
    p.add_argument('--set-name')
    p.add_argument('-k', '--ksize', default=31, type=int)
    p.add_argument('-s', '--scaled', default=1000, type=int)
    args = p.parse_args()

    mh = sourmash.MinHash(n=0, ksize=args.ksize, scaled=args.scaled)

    for line in open(args.infile):
        hashval = int(line.strip())
        mh.add_hash(hashval)

    ss = sourmash.SourmashSignature(mh, name=args.set_name)

    with sourmash_args.SaveSignaturesToLocation(args.outfile_sig) as save_sig:
        save_sig.add(ss)
        print(f'saved {len(ss)} sketches')


if __name__ == '__main__':
    sys.exit(main())
