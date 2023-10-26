import win32com.client as win32
import sys
import json


class Email:
    def __init__(self, email: dict[str, str]) -> None:
        self.to = email["to"]
        self.cc = email["cc"]
        self.subject = email["subject"]
        self.body = email["body"]

    def send_email(self) -> None:
        outlook = win32.Dispatch('outlook.application')

        mail = outlook.CreateItem(0)
        mail.To = self.to
        mail.CC = self.cc
        mail.Subject = self.subject
        mail.Body = self.body
        mail.Send()


if __name__ == "__main__":
    with open(sys.argv[1]) as jf:
        email_list = json.load(jf)
    for d in email_list:
        Email(d).send_email()

