import smtplib

from email.message import EmailMessage

with open(textfile) as fp:
    msg = EmailMessage()
    msg.set_content(fp.read())