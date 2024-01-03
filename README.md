This picture is a representation of all of the coding-sequences of all the 'canonical' transcripts in the human transcriptome. Each transcript is represented by a line in which each pixel represents a nucleotide. Longer transcripts are stacked on top of shorter transcripts with a space between each.

It's a screenshot because the full-size image is 107976x47646 (5.1 gigapixels). To display properly, it would require 29x23 4K UHD monitors. That said, it would fit on a single A0 poster provided you were able to print it at 2400x2400dpi.

The .png is only about 5Mb because of compression, but in order to open it you may require specialist software such as *vliv* or *IrfanView*.

I made this picture because I'm learning to make figures with R and ggplot2. As part of the course I'm on, there was a section on the principles and philosophy of data visualisation. This mentioned Edward Tufte and some other folk. It didn't mention Marshall McLuhan but it should've. It stressed that the representation of data should not obscure the meaning of the data. It suggested that things like log-scales may be unsuitable for non-technical audiences who might not understand them.

There's an argument to be had about whether visual representation itself can obscure meaningful concepts by imposing a sometimes inappropriate framework for their interpretation. Still, though, a good visual representation of something will tend to be better than a bad one.

I got to thinking about what kind of chart would be best to stuff 'coding-sequence-length-in-descending-order' into, in order to best convey the underlying information. And then about how well we generally unpack the informational content of *any* chart; whether we're technical or not. And then about how we will frequently stuff information into charts without stopping to look at it first.

I thought about using R to write a program to 'look at it first'. However, I worried that R wouldn't be up to the task. It might be, I don't know. But I used Rust instead. I used copilot to write the program for me, so if you like, you could consider the result 'AI-generated artwork'.

I thought about making different colours of pixel correspond to different nucleotides. However, just parsing and sorting coding-sequence-lengths and drawing lines took my laptop about twenty minutes to do. Parsing coding-sequences and drawing multicoloured lines would probably require cloud-computing or a HPC.

Anyway, it's a big picture. It's structured information with a 1:1 data:ink ratio. Printed on a poster, it would make visual sense if you looked at it under a magnifying glass, and then from a few metres away. 

It's not very intricate, but it does demonstrate the potential of exploiting fine-detail at the resolution limit of a commercial printer to communicate biological - or other - messages. Certainly with procedural or generative approaches, a lot of intricacy would be possible.



