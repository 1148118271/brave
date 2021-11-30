-- MySQL dump 10.13  Distrib 8.0.26, for macos11 (x86_64)
--
-- Host: 127.0.0.1    Database: blogs
-- ------------------------------------------------------
-- Server version	8.0.26

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `blog_details`
--

DROP TABLE IF EXISTS `blog_details`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_details` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
  `blog_info_id` int DEFAULT NULL COMMENT '博客信息id',
  `details` text COMMENT '博客的详细信息',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '修改时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客详情';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_details`
--

LOCK TABLES `blog_details` WRITE;
/*!40000 ALTER TABLE `blog_details` DISABLE KEYS */;
INSERT INTO `blog_details` VALUES (1,1,'    <p>Django 是使用 Python 编写的一个开源 Web 框架，可以用来快速搭建一个高性能的网站。</p>\n            <blockquote><p>Django makes it easier to build better Web apps more quickly and with less\n                code.</p>\n                <p>Django 让你以更快的速度、更少的代码、更轻松的方式搭建更好的 Web 应用。</p>\n            </blockquote>\n            <p>本教程将带你使用 Django 快速开发属于自己的 Blog 网站。</p>\n            <h2>教程特点</h2>\n            <p>免费、中文、零基础，完整的项目，基于最新版 Django 1.10 和 Python 3.5。</p>\n            <p>带你从零开始一步步开发属于自己的博客网站，帮助你以最快的速度掌握 Django 开发的技巧。</p>\n            <h2>谁适合这个教程</h2>\n            <p><strong>本教程主要面向零基础的 Django 新人。</strong></p>\n            <p>只需要一点点 Python 语言的基础，就可以顺利阅读本教程。</p>\n            <p>如果你已有一定的 Django 开发经验，也能从本教程中学到更多的 Django 开发技巧。</p>\n            <h2>在线预览</h2>\n            <p>点击预览：<a href=\'\'>Django Blog Demo</a></p>\n            <p><img src=\'\' alt=\'\'/></p>\n            <h2>资源列表</h2>\n            <p>项目完整代码托管在 GitHub：<a href=\'\'>Django Blog Tutorial</a></p>\n            <p>博客前端模板托管在 GitHub：<a href=\'\'>博客模板</a></p>\n            <h2>获取帮助</h2>\n            <p>在项目开发中遇到问题，即时获取帮助。</p>\n            <p>Django 学习小组 QQ 群，扫描下方二维码加入。</p>\n            <p><img src=\'\' alt=\'\'/></p>\n            <p>或者你也可以将问题的详细描述通过邮件发送至 <a href=\'mailto:djangostudyteam@163.com\' target=\'_blank\'>djangostudyteam@163.com</a>，一般会在\n                24 小时内答复。</p>','2021-11-22 11:32:04','2021-11-22 11:32:06'),(2,10,'<p>是是是\n是是是<br>\n是是是\n搜索</p>\n<p>是 </p>','2021-11-27 18:55:11','2021-11-27 18:55:11'),(3,18,'<h1 id=\"\">是是是搜索方法搜索</h1>','2021-11-27 19:25:43','2021-11-27 19:25:43'),(4,19,'<h1 id=\"centercenter\"><center> 这是标题 <center></center></center></h1>\n<p></p><center>这是内容</center><p></p>','2021-11-27 20:38:56','2021-11-27 20:38:56'),(5,20,'<h1 id=\"centercenter\"><center> 测试博客 </center></h1>\n<p></p><center> 这是一个测试博客, 暂时没有其他作用!!!! </center><p></p>','2021-11-27 20:45:03','2021-11-27 20:45:03');
/*!40000 ALTER TABLE `blog_details` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `blog_group`
--

DROP TABLE IF EXISTS `blog_group`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_group` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
  `group_key` varchar(10) DEFAULT NULL COMMENT '分组标识',
  `group_value` varchar(20) DEFAULT NULL COMMENT '分组名称',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '修改时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客分组';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_group`
--

LOCK TABLES `blog_group` WRITE;
/*!40000 ALTER TABLE `blog_group` DISABLE KEYS */;
INSERT INTO `blog_group` VALUES (1,'js','技术','2021-11-22 11:28:55','2021-11-22 11:28:58'),(2,'sh','生活','2021-11-25 18:51:45','2021-11-25 18:51:48');
/*!40000 ALTER TABLE `blog_group` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `blog_info`
--

DROP TABLE IF EXISTS `blog_info`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_info` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
  `title` varchar(256) DEFAULT NULL COMMENT '博客标题',
  `generalize` varchar(2048) DEFAULT NULL COMMENT '博客简介',
  `user_account` varchar(50) DEFAULT NULL COMMENT '发表人',
  `publish_time` datetime DEFAULT NULL COMMENT '发表时间',
  `group_id` int DEFAULT NULL COMMENT '博客分类关联id',
  `read_count` int DEFAULT NULL COMMENT '阅读次数',
  `is_publish` varchar(1) DEFAULT NULL COMMENT '是否发表 0 未发表 1 已发表',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '修改时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客列表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_info`
--

LOCK TABLES `blog_info` WRITE;
/*!40000 ALTER TABLE `blog_info` DISABLE KEYS */;
INSERT INTO `blog_info` VALUES (20,'这是测试博客','这是一个测试博客,没有其他作用...','gxk','2021-11-27 20:45:08',1,10,'1','2021-11-27 20:43:53','2021-11-27 20:43:53');
/*!40000 ALTER TABLE `blog_info` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `blog_user`
--

DROP TABLE IF EXISTS `blog_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_user` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
  `account` varchar(45) NOT NULL COMMENT '账号',
  `password` varchar(255) NOT NULL COMMENT '密码',
  `name` varchar(255) DEFAULT NULL COMMENT '用户名',
  `sex` int DEFAULT NULL COMMENT '性别 0 男 1 女',
  `age` int DEFAULT NULL COMMENT '年龄',
  `phone` varchar(20) DEFAULT NULL COMMENT '手机号',
  `email` varchar(50) DEFAULT NULL COMMENT '邮箱',
  `portrait` varchar(255) DEFAULT NULL COMMENT '头像',
  `create_time` datetime NOT NULL COMMENT '创建日期',
  `update_time` datetime DEFAULT NULL COMMENT '修改日期',
  `del` int NOT NULL DEFAULT '1' COMMENT '逻辑删除 0 删除 1未删除',
  PRIMARY KEY (`id`),
  UNIQUE KEY `blog_user_account_uindex` (`account`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb3 COMMENT='用户表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_user`
--

LOCK TABLES `blog_user` WRITE;
/*!40000 ALTER TABLE `blog_user` DISABLE KEYS */;
INSERT INTO `blog_user` VALUES (1,'gxk','e308bb9ed8e7904b1ae3cd1458475d18','子木',0,24,'13964019534','gxk_717@163.com',NULL,'2021-11-15 15:41:33','2021-11-15 15:41:35',1);
/*!40000 ALTER TABLE `blog_user` ENABLE KEYS */;
UNLOCK TABLES;



-- 2021-11-30 17:00:53 博客评论表
DROP TABLE IF EXISTS `blog_comments`;
create table blog_comments
(
    id          int auto_increment
        primary key,
    blog_id     int          not null comment '博客关联id',
    name        varchar(100) null comment '名字',
    email       varchar(200) null comment '邮箱',
    url         varchar(500) null comment '网址',
    comment     text         null comment '评论',
    create_time datetime     null comment '发表时间'
)
    comment '博客评论';
