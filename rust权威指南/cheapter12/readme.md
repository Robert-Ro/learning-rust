## 测试驱动开发(test-driven development, TDD)流程

1. 编写一个会失败的测试，运行该测试，确保它会如期运行失败
2. 编写或修改刚好足够多的代码让新测试通过
3. 在保证测试始终通过的前提下重构刚刚编写的代码
4. 返回步骤1.，进行下一轮开发

虽然测试驱动开发只是众多软件开发技术中的一个，但它能对代码的设计工作起到指导和帮助的作用。**优先编写测试，然后再编写能够通过测试的代码也有助于在开发过程中保持较高的测试覆盖率**。