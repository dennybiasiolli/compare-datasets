import sys
import pandas as pd

def compare_datasets(filepath1, filepath2, key_name):
    df1  = pd.read_csv(filepath1).set_index(key_name)
    df1.sort_index(inplace=True)
    df2  = pd.read_csv(filepath2).set_index(key_name)
    df2.sort_index(inplace=True)
    return df1.compare(df2, align_axis=1)


def main():
    filepath1 = sys.argv[1]
    filepath2 = sys.argv[2]
    key_name = sys.argv[3]

    diff = compare_datasets(filepath1, filepath2, key_name)
    print(diff)

if __name__ == "__main__":
    main()
