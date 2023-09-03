import win32com.client as win32
import sys
import json


class Email:
    def __init__(self, email: dict[str, str]) -> None:
        self.to = email["to"]
        self.cc = email["cc"]
        self.subject = email["subject"]
        self.body = email["body"]

def send_email(email: Email) -> None:
    outlook = win32.Dispatch('outlook.application')

    mail = outlook.CreateItem(0)
    mail.To = email.to
    mail.CC = email.cc
    mail.Subject = email.subject
    mail.Body = email.body
    mail.Send()


if __name__ == "__main__":
    with open(sys.argv[1]) as jf:
        email_list = json.load(jf)
    for d in email_list:
        send_email(Email(d))

