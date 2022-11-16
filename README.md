# java_authors

*java_authors* is a tool written in Rust for adding the @author tag to all your .java files.

<code>
/**<br>
*<br>
* @author {Your name here} <br>
*/
</code>

<br><br>
It will add the previous comment if @author tag is not found in the file. If it is found it will replace the author with the specified.
<br>
<br>

<h2> USE </h2>

./java_authors -a \<Author\> -p \<Project dir\>

Default project dir: "."




