# Content creator

the purpose of this app is to become a big channel fast by exploiting the algorithm and watch time

TODO:

1. ~~Get content (wisdom, just some text,..etc)~~ **DONE**
2. ~~Generate a picture with that text~~ **DONE**
3. ~~create a 5~7 sec video~~ **DONE**
4. upload it to youtube

### Getting content

I decide to use https://www.quotes.net/api.php , https://animechan.vercel.app/api/random 
, ~~ https://motivational-quote-api.herokuapp.com/quotes/random ~~ 
https://www.abbreviations.com/services/v2/quotes.php?uid=10998&tokenid=uNMIuLwQmsm1lf7W&format=json
I havent decided what to use yet

### Making Image
I have decided instead of making my own images, I can use Inspirobot to generate images

its much better and results are sarcastic and funny

### BONUS: Twitter

I have added the ability to tweet the image, It is made from scratch. Signiture ,noonce,..etc.

Iam not sure if there are any bugs but it works well with me

### Creating video

Using ffmpeg, I run the following code

`ffmpeg -framerate 1/5 -i quotes/quote.jpeg -c:v libx264 -r 30 -pix_fmt yuv420p quotes/quote.mp4`

# youtube Coming soon...