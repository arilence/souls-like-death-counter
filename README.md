# Souls-Like Death Counter

Created for [SmawlGiant's Twitch Stream](https://www.twitch.tv/smawlgiant)

This application aims to automatically track death counts among various souls-like games. The number of deaths is saved and continuously updated to a text file for easy access into OBS or other streaming softwares.

It currently works with: DARK SOULS: Prepare To Die Edition, DARK SOULS: REMASTERED, and DARK SOULS III.


## Usage

Make sure to start the game you want to track once prior to running this application so that the save file data is accessible on disk.

The first time you run the program it will generate a config file.

```
$ cargo run
Souls-Like Death Counter v0.4.0
Generating Config File!
Game Selected: Dsr
q + enter to quit
Started Successfully
```

After editing the config file to your choosing, run the program again to start counting. The deaths.txt file will be updated everytime a change is detected.


## Notes

This is a personal project to help me learn the rust programming language. Just warning ahead of time, things will be ugly.

Also, I acknowledge there are other applications which achieve a similar goal by reading the processes memory. For fun I have opted to read and parse the games save file to retrieve the death count. 
