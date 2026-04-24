import os

if __name__ == "__main__":
    var_drasi = os.getenv("VAR_DRASI")
    print(var_drasi)

    if var_drasi:
        print("VAR_DRASI is set to:", var_drasi)
    else:
        raise ValueError("VAR_DRASI environment variable is not set.")