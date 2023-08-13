import os
from resumer import Resumer

def main():
    files = os.listdir("cap")
    resumer = Resumer()

    for file_name in files:
        resumer.resume_file(file_name)

if __name__ == '__main__':
    main()