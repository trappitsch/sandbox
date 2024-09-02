from pathlib import Path

from rust_python_test._lowlevel import hello, lst_to_crd_rs

__all__ = ["hello"]

def lst_to_crd() -> None:
    """Convert a list file to a crd file.

    If the file exists, a new file will just be written with the `crd` ending. 
    Otherwise, an error will be raised in Rust and moved over here.

    :return: None

    :raises: FileNotFoundError: If the file does not exist.
    """
    tmp_path = Path(__file__).parents[2].joinpath("tmp")

    # file that exists
    file_ex = tmp_path.joinpath("exists.lst")
    file_nex = tmp_path.joinpath("not_exists.lst")

    lst_to_crd_rs(str(file_ex.absolute()))

    try:
        lst_to_crd_rs(str(file_nex.absolute()))
    except FileNotFoundError as e:
        print(e)


    print(hello())

    print("Well, not much here yet")

