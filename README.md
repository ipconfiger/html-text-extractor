# html-text-extractor
A universal web page main content extractor based on line block density distribution.

本文的提取方法基于以下文献：
《基于行块分布函数的通用网页正文抽取》是哈尔滨工业大学信息检索研究中心陈 鑫 (Xin Chen) 的研究成果，详情在这：https://code.google.com/archive/p/cx-extractor/
但是在我实际的测试中，该方法针对很多实现并不规范的网页效果并不好，因为在实测中很多网站的正文是通过JS写入某个div，然后有的网站还会随机在写入正文的div中，在后面随机插入广告。
而且某些内容，比如古龙的小说，正文非常容易就被当成超链接按钮什么的给错过去了。没错，我开始这个研究的初衷并不怎么合法，因为创世大神：河蟹 的神威，很多小说在起点上找不到了，只能在某些不怎么合法的网站上能找到，所以想要扒拉下来生成EPUB文档，就需要研究一下如何提取正文。
在经过多次尝试后，我将原方法进行了改良，在分行的时候不剔除html标签，而是逐行处理的时候根据不同的tag来处理，首先去掉SCRIPT和STYLE标签和其中的内容，然后提取BODY的内容，并组行判断行内有无A标签，如果有就进一步操作剔除A标签以及标签内含的部分，然后剔除所有其他标签，但是保留标签的内容，如果没剩下内容，那么这一行多半是导航连接什么的，标识为0。
如果行内没有A标签，就剔除所有其他标签，看看剩下的内容是否还有内容，如果没有，说明这行是纯标签，标识为0，如果有就标识为1. 之后将所有标识为1的行的下标序号存到一个数组内。
然后使用滑动窗口法将保存序号的数组进行分组，这个操作就等于是剔除导航区，侧边栏菜单，底部导航链接区将页面内容进行区块划分。用滑动窗口的原因是某些生成不规范的html，会将P标签搞成
```
<p>《基于行块分布函数的通用网页正文抽取》是哈尔滨工业大学信息检索研究中心陈 鑫 (Xin Chen) 的研究成果
</p></p>
详情在这：https://code.google.com/archive/p/cx-extractor/
</p><p>
```
这种形式，如果按照之前的方式，就会造成序号并不连续，用滑动窗口可以弥合这个问题。
此时按照标识1提取就能得到： [1, 5, 7, 8, 33, 35, 37, 38, 39, 41, 43, 45, 77, 78] 这样的数组，按滑动窗口分块就会得到  [[1], [5, 7, 8], [33, 35, 37, 38, 39, 41, 43, 45], [77, 78]] 这样的数组。这时候将所有块内的行，去掉html标签，获取长度并加到一起作为这一个块的权重，值最大的块，就是正文部分了。
为什么不直接取最长的块？因为有一些网站会通过js将html写入某个div，写入的html是处理过的，就只有一行，那么这个时候有可能得到的数组就是：
[1,3,7,8,9,10,56,78,79], 这样分块后 [[1,3],[7,8,9,10],[56],[78,79]], 其实 [56] 才是正文，但是因为只有一行，所以被滤掉了。所以最终还是要靠正文的长度来作为权重。

将正文部分区块处理后合并成一个字符串返回，就完成了正文提取的工作。

因为我实际使用的时候是Rust写的，所以用了Rust版本，我这里再用Python实现了一个版本出来，方便大家理解。

## English description

The extraction method presented in this paper is based on the following literature:

“Universal Web Page Main Content Extraction Based on Line Block Distribution Function” is the research outcome of Xin Chen from the Information Retrieval Research Center of Harbin Institute of Technology. More details can be found here: https://code.google.com/archive/p/cx-extractor/

However, in my practical testing, this method does not perform well for many web pages that are not implemented according to standards. This is because, in actual measurements, the main content of many websites is written into a div via JavaScript, and some websites also randomly insert advertisements after the div containing the main content.

Moreover, certain content, such as novels by Gu Long, is often mistakenly identified as hyperlinks or buttons. Indeed, the initial motivation for starting this research was not entirely legal, as many novels are no longer available on Qidian due to the influence of the Great Firewall. They can only be found on some less-than-legal websites. Therefore, to scrape and generate EPUB documents, it is necessary to study how to extract the main content.

After several attempts, I improved the original method by not removing HTML tags during line breaks but processing each line individually based on different tags. First, I removed the SCRIPT and STYLE tags and their contents, then extracted the content of the BODY tag, and judged whether there were A tags within the lines. If there were, further operations were performed to remove the A tags and their contents. Then, all other tags were removed while retaining their content. If no content remained, the line was likely a navigation link and was marked as 0.

If there were no A tags in the line, all other tags were removed to see if any content remained. If not, the line was marked as purely containing tags and was also marked as 0. If content remained, it was marked as 1. All lines marked as 1 had their indices stored in an array.

The array of indices was then grouped using a sliding window method, which effectively removed navigation bars, side menu bars, and footer navigation link areas, dividing the page content into blocks. The use of a sliding window is necessary because some non-standard HTML generation can result in irregular structures, such as:

```
<p>Universal Web Page Main Content Extraction Based on Line Block Distribution Function
</p></p>
Details here: [https://code.google.com/archive/p/cx-extractor/](https://code.google.com/archive/p/cx-extractor/)
</p><p>

```
If processed in the previous manner, this would result in non-continuous indices. The sliding window method helps to bridge this issue.

At this point, by extracting lines marked as 1, we can obtain an array like [1, 5, 7, 8, 33, 35, 37, 38, 39, 41, 43, 45, 77, 78]. Grouping by the sliding window method results in arrays like [[1], [5, 7, 8], [33, 35, 37, 38, 39, 41, 43, 45], [77, 78]]. Each block’s lines have their HTML tags removed, their lengths are measured, and the total is used as the block’s weight. The block with the highest weight is considered the main content.

Why not simply take the longest block? Because some websites use JavaScript to write HTML into a div, and the written HTML is processed, resulting in only one line. In such cases, the resulting array might be [1, 3, 7, 8, 9, 10, 56, 78, 79], which, when divided into blocks, becomes [[1, 3], [7, 8, 9, 10], [56], [78, 79]]. The actual main content might be [56], but because it is only one line, it gets filtered out. Therefore, the length of the main content must be used as the weight.

After processing the main content blocks and merging them into a single string, the extraction of the main content is complete.

Since I used Rust for my implementation, I have also provided a Python version here for easier understanding.

